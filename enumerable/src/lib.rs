//! This is a simple ERC721 contract that keeps track of the number of NFTs owned by each address.
//!
//! The interface and interface documentation was copied from:
//! https://github.com/ethereum/ERCs/blob/47d680dcebb1fa4dcfd0287c2dafcf83e9864ca0/ERCS/erc-721.md
//! Used under the Creative Commons CC0 1.0 Universal license.

#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
enum Key {
    AllTokens(/* index */ u32),
    AllTokensLen,
    Token(/* token_id */ u32),
    OwnedTokens(/* owner */ Address, /* index */ u32),
    OwnedTokensLen(/* owner */ Address),
}

#[contracttype]
struct Token {
    owner: Address,
    all_index: u32,
    owned_index: u32,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn mint(e: &Env, to: Address, token_id: u32) {
        if let Some(_) = e
            .storage()
            .persistent()
            .get::<_, Address>(&Key::Token(token_id))
        {
            panic!("Token already exists");
        }

        let all_tokens_len = e
            .storage()
            .persistent()
            .get(&Key::AllTokensLen)
            .unwrap_or(0);
        let all_index = all_tokens_len;
        e.storage()
            .persistent()
            .set(&Key::AllTokens(all_index), &token_id);
        e.storage()
            .persistent()
            .set(&Key::AllTokensLen, &(all_tokens_len + 1));

        let owned_tokens_len = e
            .storage()
            .persistent()
            .get(&Key::OwnedTokensLen(to.clone()))
            .unwrap_or(0);
        let owned_index = owned_tokens_len;
        e.storage()
            .persistent()
            .set(&Key::OwnedTokens(to.clone(), owned_index), &token_id);
        e.storage()
            .persistent()
            .set(&Key::OwnedTokensLen(to.clone()), &(owned_tokens_len + 1));

        e.storage().persistent().set(
            &Key::Token(token_id),
            &Token {
                owner: to,
                all_index,
                owned_index,
            },
        );
    }

    /// Count of all tokens.
    pub fn total_supply(e: &Env) -> u32 {
        e.storage()
            .persistent()
            .get(&Key::AllTokensLen)
            .unwrap_or(0)
    }

    /// Token at the given index.
    ///
    /// To get the count of all tokens, call `total_supply`.
    pub fn token_by_index(e: &Env, index: u32) -> u32 {
        e.storage()
            .persistent()
            .get(&Key::AllTokens(index))
            .unwrap()
    }

    /// Count of all tokens assigned to an owner.
    pub fn balance(e: &Env, owner: Address) -> u32 {
        e.storage()
            .persistent()
            .get(&Key::OwnedTokensLen(owner))
            .unwrap_or(0)
    }

    /// Token owned by the owner, at the given index.
    ///
    /// To get the count of all tokens owned by the owner, call `balance`.
    pub fn token_of_owner_by_index(e: &Env, owner: Address, index: u32) -> u32 {
        e.storage()
            .persistent()
            .get(&Key::OwnedTokens(owner, index))
            .unwrap()
    }

    /// Owner of the token.
    pub fn owner(e: &Env, token_id: u32) -> Address {
        e.storage()
            .persistent()
            .get::<_, Token>(&Key::Token(token_id))
            .unwrap()
            .owner
    }

    /// Transfer ownership of an NFT from one address to another address.
    pub fn transfer(e: &Env, from: Address, to: Address, token_id: u32) {
        // Get the token.
        let token = e
            .storage()
            .persistent()
            .get::<_, Token>(&Key::Token(token_id))
            .unwrap();

        // Check if from is the owner.
        if token.owner != from {
            panic!("From is not the owner");
        }

        // Remove token from's owned tokens, by swapping the last token into the removed tokens
        // slot.
        let from_tokens_len = e
            .storage()
            .persistent()
            .get::<_, u32>(&Key::OwnedTokensLen(from.clone()))
            .unwrap();
        let from_last_index = from_tokens_len - 1;
        let from_last_token_id = e
            .storage()
            .persistent()
            .get::<_, u32>(&Key::OwnedTokens(from.clone(), from_last_index))
            .unwrap();
        e.storage()
            .persistent()
            .remove(&Key::OwnedTokens(from.clone(), from_last_index));
        e.storage()
            .persistent()
            .set(&Key::OwnedTokensLen(from.clone()), &(from_tokens_len - 1));
        if from_last_index != 0 {
            e.storage().persistent().set(
                &Key::OwnedTokens(from.clone(), token.owned_index),
                &from_last_token_id,
            );
        }

        // Add token to to's owned tokens.
        let to_tokens_len = e
            .storage()
            .persistent()
            .get(&Key::OwnedTokensLen(to.clone()))
            .unwrap_or(0);
        let to_index = to_tokens_len;
        e.storage()
            .persistent()
            .set(&Key::OwnedTokens(to.clone(), to_index), &token_id);
        e.storage()
            .persistent()
            .set(&Key::OwnedTokensLen(to.clone()), &(to_tokens_len + 1));

        // Update token.
        e.storage().persistent().set(
            &Key::Token(token_id),
            &Token {
                owner: to,
                all_index: token.all_index,
                owned_index: to_index,
            },
        );
    }
}

mod test;

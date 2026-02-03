default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean

CSV_FILE := costs.csv

# Deploy to testnet and invoke each function with --cost, recording to CSV
deploy-and-cost: build
	@echo "=== Deploying ==="
	stellar contract deploy \
		--wasm target/wasm32v1-none/release/soroban_nft_contract.wasm \
		--alias token-id-contract
	@echo ""
	@echo "=== Recording costs to $(CSV_FILE) ==="
	@echo "function,fee_proposed,inclusion_fee,resource_fee,non_refundable,refundable,inclusion_fee_charged,non_refundable_charged,refundable_charged,resource_fee_charged,fee_charged,refunded" > $(CSV_FILE)
	@for fn_call in \
		"store_u32 --token_id 42:store_u32" \
		"load_u32:load_u32" \
		"event_u32 --token_id 42:event_u32" \
		"store_u256 --token_id 42:store_u256" \
		"load_u256:load_u256" \
		"event_u256 --token_id 42:event_u256"; \
	do \
		FN_ARGS=$${fn_call%%:*}; \
		FN_NAME=$${fn_call##*:}; \
		echo "Invoking $$FN_NAME..."; \
		OUTPUT=$$(stellar contract invoke --id token-id-contract --cost -- $$FN_ARGS 2>&1); \
		FEE_PROPOSED=$$(echo "$$OUTPUT" | grep "Fee Proposed:" | sed 's/.*Fee Proposed: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		INCLUSION_FEE=$$(echo "$$OUTPUT" | grep "Inclusion Fee:" | head -1 | sed 's/.*Inclusion Fee: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		RESOURCE_FEE=$$(echo "$$OUTPUT" | grep "Resource Fee:" | head -1 | sed 's/.*Resource Fee: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		NON_REFUNDABLE=$$(echo "$$OUTPUT" | grep "Non-Refundable:" | head -1 | sed 's/.*Non-Refundable: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		REFUNDABLE=$$(echo "$$OUTPUT" | grep "Refundable:" | head -1 | sed 's/.*Refundable: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		INCLUSION_FEE_CHARGED=$$(echo "$$OUTPUT" | grep "Inclusion Fee Charged:" | sed 's/.*Inclusion Fee Charged: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		NON_REFUNDABLE_CHARGED=$$(echo "$$OUTPUT" | grep "Non-Refundable Charged:" | sed 's/.*Non-Refundable Charged: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		REFUNDABLE_CHARGED=$$(echo "$$OUTPUT" | grep "Refundable Charged:" | sed 's/.*Refundable Charged: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		RESOURCE_FEE_CHARGED=$$(echo "$$OUTPUT" | grep "Resource Fee Charged:" | sed 's/.*Resource Fee Charged: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		FEE_CHARGED=$$(echo "$$OUTPUT" | grep "Fee Charged:" | sed 's/.*Fee Charged: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		REFUNDED=$$(echo "$$OUTPUT" | grep "Refunded:" | head -1 | sed 's/.*Refunded: *\([0-9]*\).*/\1/' | tr -d '\n'); \
		echo "$$FN_NAME,$$FEE_PROPOSED,$$INCLUSION_FEE,$$RESOURCE_FEE,$$NON_REFUNDABLE,$$REFUNDABLE,$$INCLUSION_FEE_CHARGED,$$NON_REFUNDABLE_CHARGED,$$REFUNDABLE_CHARGED,$$RESOURCE_FEE_CHARGED,$$FEE_CHARGED,$$REFUNDED" >> $(CSV_FILE); \
		echo "  Fee Charged: $$FEE_CHARGED, Refunded: $$REFUNDED"; \
	done
	@echo ""
	@echo "=== Results ==="
	@cat $(CSV_FILE)

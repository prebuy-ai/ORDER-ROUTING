# Order Routing Location Rule - Route by Line Item Properties

This Shopify Function extension implements custom order routing logic that routes line items to specific locations based on line item properties.

## How it Works

When an order is placed and includes a line item with a custom property like `_locationId: 99997811016`, this function will:

1. Parse the line item properties to find any `_locationId` attributes
2. Match the location ID value to the actual location handle using the locations data
3. Prioritize the specified location for fulfillment by giving it rank 0 (highest priority)
4. Give all other locations rank 1 (lower priority)

## Line Item Property Format

The function looks for line item properties with:
- **Key**: `_locationId` 
- **Value**: The numeric location ID (e.g., `99997811016`)

## Example Scenario

If a cart contains:
- Line item with property `_locationId: 99997811016`
- Available locations: Location A (ID: 99997811016), Location B (ID: 99997811017), Location C (ID: 99997811018)

The function will:
- Give Location A (handle corresponding to ID 99997811016) rank 0 (highest priority)
- Give Location B and C rank 1 (lower priority)
- Shopify will fulfill from Location A first

## Installation

1. Install Rust and the wasm32-wasip1 target:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-wasip1
   ```

2. Build the function:
   ```bash
   shopify app function build
   ```

3. Deploy the app:
   ```bash
   shopify app deploy
   ```

## Testing

Run the tests:
```bash
cargo test
```

Test with sample input:
```bash
shopify app function run --input=input.json --export=cart_fulfillment_groups_location_rankings_generate_run
```

## File Structure

- `src/cart_fulfillment_groups_location_rankings_generate_run.rs` - Main function logic
- `src/cart_fulfillment_groups_location_rankings_generate_run.graphql` - GraphQL input query
- `src/main.rs` - Entry point and schema definitions
- `input.json` - Sample input for testing
- `expected-output.json` - Expected output for the sample input

## Configuration

This function requires:
- Shopify Plus plan (for custom order routing location rules)
- Line items with `_locationId` properties set at the storefront/cart level
- Valid location IDs that correspond to actual store locations

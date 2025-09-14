# Force Location Assignment - Complete Solution

## Problem
The location routing function was working correctly (giving the right location rank 0), but Shopify's default behavior respects inventory availability - it won't route to a location without stock, even with highest priority.

## Solution: Two-Function Approach

We now have **two complementary functions** that work together:

### 1. **Location Routing Function** (Existing)
- **Purpose**: Prioritizes locations based on `_locationId` properties
- **Location**: `extensions/location-routing-by-properties/`
- **What it does**: Gives preferred location rank 0 (highest priority)

### 2. **Fulfillment Constraints Function** (New)
- **Purpose**: **Forces** items to be fulfilled from specific locations
- **Location**: `extensions/force-location-constraints/`
- **What it does**: Creates fulfillment constraints that **override inventory availability**

## How It Works Together

1. **Line item** has property `_locationId: 108050547019`
2. **Location Routing Function** runs → prioritizes location `108050547019`
3. **Fulfillment Constraints Function** runs → **forces** items with location properties to be fulfilled together
4. **Result**: Item MUST be fulfilled from `108050547019`, even if no inventory

## Function Execution Order

```
Order Placed
     ↓
Location Routing Function (prioritizes location)
     ↓  
Fulfillment Constraints Function (forces location)
     ↓
Shopify fulfills from specified location (ignoring inventory)
```

## Deployment Steps

1. **Build both functions**:
   ```bash
   shopify app function build
   # Select: location-routing-by-properties
   shopify app function build  
   # Select: force-location-constraints
   ```

2. **Deploy the app**:
   ```bash
   shopify app dev
   ```

3. **Configure in Shopify Admin**:
   - Go to Settings → Shipping and delivery → Order routing
   - **Add location rule**: Select `location-routing-by-properties` (drag to top)
   - **Add fulfillment constraint**: Select `force-location-constraints`

## Testing

With both functions active:
- Add item with `_locationId: 108050547019` property
- Complete checkout
- Order will be assigned to location `108050547019` **regardless of inventory**

## Key Benefits

✅ **Overrides inventory**: Items route to specified location even with zero stock  
✅ **Maintains flexibility**: Items without properties use normal routing  
✅ **Prevents split fulfillment**: All items with same location ID stay together  
✅ **Backward compatible**: Existing orders without properties work normally  

## Important Notes

- **Shopify Plus required**: Custom order routing only works on Plus plans
- **Both functions needed**: Location routing alone respects inventory limits
- **Fulfillment constraints**: This is what actually forces the location assignment
- **Priority order**: Make sure location routing function is highest priority rule

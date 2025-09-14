# Testing Guide: Order Routing by Line Item Properties

## Prerequisites

✅ App installed on development store  
✅ Function built and deployed  

## Step 1: Configure Order Routing Rule

1. Go to your Shopify Admin: `https://prebuy-test.myshopify.com/admin`
2. Navigate to **Settings** → **Shipping and delivery**
3. Scroll to **Order routing** section → Click **Manage**
4. Click **Add rule**
5. Select **location-routing-by-properties** function
6. **Drag it to the top** to make it highest priority
7. Click **Save**

## Step 2: Set Up Test Locations

You need at least 2 locations for testing:

1. Go to **Settings** → **Locations**
2. Note down the **Location IDs** (you'll need these)
3. To find Location IDs:
   - Go to a location's edit page
   - Look at the URL: `https://prebuy-test.myshopify.com/admin/locations/12345678`
   - The number at the end (12345678) is your Location ID

## Step 3: Create Test Products

1. Create a test product with inventory at multiple locations
2. Make sure it's available for online sales

## Step 4: Test the Function

### Test Case 1: With Location Property

Add to cart with JavaScript in browser console:

```javascript
// Replace with your actual variant ID and location ID
fetch('/cart/add.js', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    'items': [{
      'id': 'YOUR_VARIANT_ID',  // Replace with actual variant ID
      'quantity': 1,
      'properties': {
        '_locationId': 'YOUR_LOCATION_ID'  // Replace with actual location ID
      }
    }]
  })
})
.then(response => response.json())
.then(data => console.log('Added to cart:', data));
```

### Test Case 2: Without Location Property (Control)

```javascript
fetch('/cart/add.js', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    'items': [{
      'id': 'YOUR_VARIANT_ID',  // Same variant ID
      'quantity': 1
      // No properties - should use default routing
    }]
  })
})
.then(response => response.json())
.then(data => console.log('Added to cart:', data));
```

## Step 5: Complete Checkout and Verify

1. **Complete the checkout** for both test cases
2. **Check the orders** in Admin → Orders
3. **Verify fulfillment location**:
   - Order with `_locationId` should be assigned to specified location
   - Order without property should use default location

## Step 6: Debug Function Execution

Monitor function execution:

```bash
# In terminal, run the dev server and watch for function logs
shopify app dev
```

Look for function execution logs when orders are placed.

## Expected Results

### With Location Property:
- ✅ Order routes to specified location (Location ID in property)
- ✅ Function gives that location rank 0 (highest priority)

### Without Location Property:
- ✅ Order routes using default Shopify logic
- ✅ All locations get equal priority (rank 0)

## Troubleshooting

### Function not executing?
- Check order routing rules are enabled and prioritized correctly
- Ensure you have Shopify Plus (required for custom location rules)

### Wrong location selected?
- Verify Location ID in property matches actual location
- Check that location has inventory for the product
- Ensure location is active and can fulfill orders

### Property not being read?
- Verify property key is exactly `_locationId` (case sensitive)
- Check property value is numeric string (e.g., "12345678")
- Ensure property is set on line item, not cart level

## Quick Test Script

Run this in your browser console on the cart page:

```javascript
// Get your location IDs first
console.log('Current cart:', await fetch('/cart.js').then(r => r.json()));

// Test with location preference
await fetch('/cart/clear.js', {method: 'POST'});
await fetch('/cart/add.js', {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    items: [{
      id: 'YOUR_VARIANT_ID',
      quantity: 1,
      properties: {'_locationId': 'YOUR_LOCATION_ID'}
    }]
  })
});

console.log('Cart with location preference:', await fetch('/cart.js').then(r => r.json()));
```

Replace `YOUR_VARIANT_ID` and `YOUR_LOCATION_ID` with actual values from your store.

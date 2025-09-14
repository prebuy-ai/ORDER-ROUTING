// Example: How to add the _locationId property to cart line items
// This would typically be done in your storefront theme or cart implementation

// Example 1: Using Shopify Ajax Cart API
function addProductWithLocationProperty(variantId, locationId) {
  const formData = {
    'items': [{
      'id': variantId,
      'quantity': 1,
      'properties': {
        '_locationId': locationId
      }
    }]
  };

  fetch('/cart/add.js', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(formData)
  })
  .then(response => response.json())
  .then(data => {
    console.log('Added to cart with location preference:', data);
  })
  .catch((error) => {
    console.error('Error:', error);
  });
}

// Example 2: Adding property to existing cart line
function updateCartLineWithLocation(lineKey, locationId) {
  const formData = {
    'id': lineKey,
    'properties': {
      '_locationId': locationId
    }
  };

  fetch('/cart/change.js', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(formData)
  })
  .then(response => response.json())
  .then(data => {
    console.log('Updated cart line with location preference:', data);
  })
  .catch((error) => {
    console.error('Error:', error);
  });
}

// Example 3: Using in a product form
// Add this to your product form in the theme
/*
<form action="/cart/add" method="post" enctype="multipart/form-data">
  <!-- Regular product fields -->
  <input type="hidden" name="id" value="{{ product.selected_or_first_available_variant.id }}">
  <input type="number" name="quantity" value="1" min="1">
  
  <!-- Location preference property -->
  <input type="hidden" name="properties[_locationId]" value="99997811016">
  
  <button type="submit">Add to Cart</button>
</form>
*/

// Example usage:
// addProductWithLocationProperty(39897499729985, '99997811016');
// updateCartLineWithLocation('39897499729985:abc123', '99997811017');

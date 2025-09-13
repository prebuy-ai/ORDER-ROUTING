# Prebuy Order Routing - Shopify Functions

A Shopify app that implements intelligent order routing using Shopify Functions to route orders based on line item properties and enforce location-specific fulfillment constraints.

## 🚀 Overview

This app contains two complementary Shopify Functions that work together to provide complete control over order routing and fulfillment:

### 1. **Location Routing by Properties**
- **Purpose**: Routes orders to specific locations based on line item properties
- **Location**: `extensions/location-routing-by-properties/`
- **Function**: Prioritizes locations based on `_Location ID` properties in cart items

### 2. **Force Location Constraints** 
- **Purpose**: Enforces fulfillment constraints to override inventory availability
- **Location**: `extensions/force-location-constraints/`
- **Function**: Forces items to be fulfilled from specific locations regardless of stock levels

## 🔧 How It Works

### Complete Flow
1. **Line item** contains property `_Location ID: 108050547019`
2. **Location Routing Function** runs → gives location `108050547019` rank 0 (highest priority)
3. **Fulfillment Constraints Function** runs → creates constraints to force fulfillment from specified location
4. **Result**: Item MUST be fulfilled from `108050547019`, even if inventory is unavailable

### Function Execution Order
```
Order Placed
    ↓
Location Routing Function (Rank 0 to preferred location)
    ↓
Fulfillment Constraints Function (Force fulfillment grouping)
    ↓
Shopify Fulfillment (Respects constraints)
```

## 📋 Features

- **Property-Based Routing**: Routes based on `_Location ID` line item properties
- **Inventory Override**: Forces fulfillment even when location has no inventory
- **Multi-Location Support**: Handles complex multi-location scenarios
- **Shopify Markets Compatible**: Works with international markets and locations
- **Debug-Friendly**: Comprehensive logging for troubleshooting

## 🛠️ Installation & Setup

### Prerequisites
- Shopify Partner account
- Development store or Shopify Plus sandbox
- Node.js installed
- Shopify CLI installed

### Installation
```bash
# Clone the repository
git clone https://github.com/prebuy-ai/ORDER-ROUTING.git
cd ORDER-ROUTING

# Install dependencies
npm install

# Start development server
npm run dev
```

### Deployment
```bash
# Build the functions
npm run build

# Deploy to Shopify
npm run deploy
```

## 📁 Project Structure

```
ORDER-ROUTING/
├── extensions/
│   ├── location-routing-by-properties/     # Location routing function
│   │   ├── src/
│   │   │   ├── main.rs                     # Main routing logic
│   │   │   └── *.graphql                   # GraphQL queries
│   │   ├── schema.graphql                  # Function schema
│   │   ├── shopify.extension.toml          # Extension config
│   │   └── README.md                       # Function documentation
│   │
│   └── force-location-constraints/         # Fulfillment constraints function
│       ├── src/
│       │   ├── main.rs                     # Constraint logic
│       │   └── *.graphql                   # GraphQL queries
│       ├── schema.graphql                  # Function schema
│       ├── shopify.extension.toml          # Extension config
│       └── locales/                        # Localization files
│
├── package.json                            # App configuration
├── shopify.app.toml                        # Shopify app config
├── FORCE_LOCATION_SOLUTION.md              # Solution overview
├── TESTING_GUIDE.md                        # Testing instructions
├── SECURITY.md                             # Security information
├── .gitignore                              # Git ignore rules
├── .gitattributes                          # Git attributes
└── README.md                               # This file
```

## 🔍 Line Item Property Format

The functions expect line items to have properties in this format:

```
Property Key: "_Location ID"
Property Value: "108050547019" (numeric location ID)
```

### Example Cart Item
```javascript
{
  "properties": {
    "_Location ID": "108050547019"
  }
}
```

## 🧪 Testing

See [TESTING_GUIDE.md](TESTING_GUIDE.md) for comprehensive testing instructions.

### Quick Test
1. Create a cart with line item property `_Location ID: [your-location-id]`
2. Place order
3. Check fulfillment routing in Shopify admin
4. Verify order is routed to specified location

## 📖 Documentation

- **[Force Location Solution](FORCE_LOCATION_SOLUTION.md)**: Complete solution overview
- **[Testing Guide](TESTING_GUIDE.md)**: Step-by-step testing instructions
- **[Security](SECURITY.md)**: Security considerations
- **[Location Routing README](extensions/location-routing-by-properties/README.md)**: Location routing function details

## 🔧 Configuration

### App Configuration
Edit `shopify.app.toml` to configure:
- App name and handle
- Scopes and permissions
- Extension settings

### Function Configuration
Each function has its own `shopify.extension.toml` file for:
- Function name and description
- Input/output schema
- Runtime settings

## 🚨 Troubleshooting

### Common Issues

1. **Function not triggering**
   - Verify app is installed and functions are deployed
   - Check line item properties are correctly formatted
   - Ensure locations exist and are active

2. **Routing not working**
   - Confirm `_Location ID` matches actual location ID in Shopify
   - Check function logs in Shopify Partner Dashboard
   - Verify both functions are deployed and active

3. **Inventory issues**
   - Force Location Constraints function should override inventory
   - Check constraint generation in function logs
   - Ensure fulfillment service is configured correctly

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

For support and questions:
- Check the documentation in each extension's README
- Review the testing guide
- Check Shopify Function logs in Partner Dashboard
- Open an issue in this repository

---

Built with ❤️ for intelligent order routing on Shopify
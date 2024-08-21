//! 50 lines 35 code 8 comments 7 blanks
/*
Bicep is a declarative language, which means the elements can appear in any order. Unlike imperative languages, the order of elements doesn't affect how deployment is processed.
This means you can define resources, variables, and parameters in any order you like.
*/

metadata description = 'Creates a storage account and a web app'

@description('The prefix to use for the storage account name.')
@minLength(3)
@maxLength(11)
param storagePrefix string

param storageSKU string = 'Standard_LRS'
param location string = resourceGroup().location

var uniqueStorageName = '${storagePrefix}${uniqueString(resourceGroup().id)}'

var objectExmaple = {
    name: 'John'
    age: 30
    address: '''
        1 Microsoft Way
        Redmond, WA 98052
    '''
}

// Create a storage account
resource stg 'Microsoft.Storage/storageAccounts@2022-09-01' = {
    name: uniqueStorageName
    location: location
    sku: {
        name: storageSKU
    }
    kind: 'StorageV2'
    properties: {
        supportsHttpsTrafficOnly: true
    }
}

// Use a module to deploy a web app
// Modules are a way to encapsulate and reuse resources in Bicep
module webModule './webApp.bicep' = {
    name: 'webDeploy'
    params: {
        skuName: 'S1'
        location: location
        personalInfo: objectExmaple
    }
}

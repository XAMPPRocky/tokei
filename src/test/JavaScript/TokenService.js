function TokenService ($window) {
  var tokenKey = 'Token'
  function getToken () {
    return $window.localStorage.getItem(tokenKey)
  }

  function setToken (token) {
    $window.localStorage.setItem(tokenKey, token)
    console.log('Token set: ', token)
    tokenService.token = getToken()
  }

  function tokenExists () {
    return getToken() !== null
  }

  var tokenService = { getToken : getToken
                     , setToken : setToken
                     , tokenExists : tokenExists
                     , token : getToken()
                     }

  return tokenService
}

angular.module('Echo')
       .factory('TokenService', TokenService) 

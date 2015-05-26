function UserService ($location, jwtHelper, TokenService) {
  var user
    , friendID = ''

  function createUser (token) {
    user = jwtHelper.decodeToken(token)
    var event = new CustomEvent('userCreated', {detail : user})
    document.dispatchEvent(event)
  }

  function getUser () {
    return user
  }

  function getUserID () {
    return user._id
  }

  function setFriend(newFriendID) {
    friendID = newFriendID
  }

  function getFriend () {
    return friendID
  }

  function loginEvent (token) {
    createUser(token)
    TokenService.setToken(token)
    socket.emit('logged-in', getUserID())
  }
  /*
    Hi

    This should be 5

    Thanks
  */
  function loginSuccess(response) {
    loginEvent(response.token)
    $location.path('/dashboard')
  }

  function loginError (response) {
    // TODO show errors to user
    console.log(response.status)
  }
  return { createUser : createUser
         , loginSuccess : loginSuccess
         , loginError : loginError
         , loginEvent : loginEvent
         , getUser : getUser
         , getUserID : getUserID
         , setFriend : setFriend
         , getFriend : getFriend
         }
}

angular.module('Echo')
      .factory('UserService', UserService)

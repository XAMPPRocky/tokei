function APIService ($http, $resource, TokenService) {

  var usersRoute = '/api/users/'

  var User = $resource( usersRoute
                      , {} 
                      , { signin : { method : 'POST'
                                   , url : usersRoute+'signin'
                                   }

                        , update : { method : 'PUT'
                                   }        

                        , add : { method: 'POST'
                                , url : usersRoute+'add'
                                }

                        , accept : { method: 'POST'
                                   , url : usersRoute+'accept'
                                   }

                        , remove : { method : 'POST'
                                   , url : usersRoute+'remove'
                                   }         

                        , validate : { method : 'GET'
                                     , url : usersRoute+'validate'
                                     }
                        }
                      )
  return User
}

angular.module('Echo')
       .factory('API', APIService)

function CallService (UserService) {
  var webrtc 

  function startCall(ID) {
    webrtc = new SimpleWebRTC({
      localVideoEl: 'local',
      remoteVideosEl: 'remotes',
      autoRequestMedia: true
    })

    webrtc.on('readyToCall', function () {
      webrtc.joinRoom(ID)
    })
  }

  function endCall() {
    webrtc.disconnect()
  }

  function sendOffer() {
    console.log(UserService.getUser())
    console.log(UserService.getUserID())
    socket.emit('callOffer', UserService.getUserID(), UserService.getFriend())

    socket.on('callAccepted', function (receiverID) {
      startCall(receiverID)
    })

    socket.on('callRejected', function (receiverID) {
      // TODO Replace in the UI
      alert('User is busy,')
    })
  }

  socket.on('callOffer', function (senderID) {
    console.log('Call Offered from: '+senderID)
    // TODO Replace within the UI
    var callConfirm = confirm(senderID+' is calling you. Accept?')

    if (callConfirm) {
      socket.emit('callAccepted', senderID, UserService.getUserID())
      startCall(UserService.getUserID())
    }
    else {
      socket.emit('callRejected', senderID, UserService.getID())
    }
  })

  return { sendOffer : sendOffer
         , endCall : endCall
         }
}

angular.module('Echo')
.factory('CallService', CallService)

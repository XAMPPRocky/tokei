;; 11 lines 7 code 2 comments 2 blanks
;; This is a sample taken from the official website @ fennel-lang.org

(local dirs {:up [0 -1] :down [0 1] :left [-1 0] :right [1 0]})
  
(each [key [dx dy] (pairs dirs)]
  (when (love.keyboard.isDown key)
    (let [[px py] player
          x (+ px (* dx player.speed dt))
          y (+ py (* dy player.speed dt))]
      (world:move player x y))))

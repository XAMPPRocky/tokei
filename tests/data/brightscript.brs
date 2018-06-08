' 26 lines 10 code 13 comments 3 blanks
' /**
'  * @member difference
'  * @memberof module:rodash
'  * @instance
'  * @description Return a new array of items from the first which are not in the second.
'  * @param {Array} first
'  * @param {Array} second
'  * @example

REM  * difference = _.difference([1,2], [2])
REM  * ' => [1]
REM  *
REM  */

Function rodash_difference_(first, second)
  result = []
  for each f in first
    result.push(f) 'Push array
    for each s in second
      if m.equal(s,f) then result.pop()
    end for
  end for

  return result
End Function
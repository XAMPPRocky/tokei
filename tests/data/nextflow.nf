/* 18 lines 10 code 5 comments 3 blanks */

/*
Nextflow - hello
*/

// comment
cheers = Channel.from 'Bonjour', 'Ciao', 'Hello', 'Hola'

process sayHello {
  echo true
  input: 
    val x from cheers
  script:
    """
    echo '$x world!'
    """
}

// 42 lines 28 code 5 comments 9 blanks

/*
 * Multi-line block comment
 */

// Single line comment
nextflow.enable.dsl = 2

params.greeting = 'Hello'
params.name = "World"

process sayHello {
    input:
    val greeting
    val name

    output:
    stdout

    script:
    """
    echo '${greeting}, ${name}!'
    echo "// this is not a comment"
    echo '/* also not a comment */'
    """
}

process farewell {
    output:
    stdout

    script:
    '''
    echo 'Goodbye!'
    '''
}

workflow {
    sayHello(params.greeting, params.name) | view
    farewell() | view
}

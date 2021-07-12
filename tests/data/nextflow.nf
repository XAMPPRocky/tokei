/* 35 lines 18 code 6 comments 11 blank */

/*
Nextflow - Your first script
https://www.nextflow.io/docs/latest/getstarted.html
*/

// Script parameters
params.str = 'Hello world!'

process splitLetters {

    output:
    file 'chunk_*' into letters

    """
    printf '${params.str}' | split -b 6 - chunk_
    """
}

process convertToUpper {

    input:
    file x from letters.flatten()

    output:
    stdout result

    """
    cat $x | tr '[a-z]' '[A-Z]'
    """
}

result.view { it.trim() }

// 23 lines 13 code 4 comments 6 blanks  

class Test {
    
    static def void main(String[] args) {        
        /*
         * Multiline comment
         */
        val f = new Foo()
        f.bar() // Not counted
    }
    
}

class Foo {
    
    def bar() {
        println('string type 1')
        println("string type 2")
        println('''string type 3''')
    }
    
}

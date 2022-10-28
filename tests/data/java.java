/* 37 lines 23 code 5 comments 9 blanks */

/*
 * Simple test class
 */
public class Test
{
    int j = 0; // Not counted
    public static void main(String[] args)
    {
        Foo f = new Foo();
        f.bar();
        
    }
}

class Foo
{
    public void bar()
    {
      System.out.println("FooBar"); //Not counted
    }
}

// issues/915
public class BackSlash {
    public void run()
    {
      "\\"; // 1 code + 2 blanks


      "\\"; // 1 code + 3 blanks



    }
}

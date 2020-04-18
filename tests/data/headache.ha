// 13 lines 9 code 3 comments 1 blanks
/* //The Headache cat program */
// Implementation of Unix Cat in Headache

void main() {
    char x;
    read x; //read from stdin
    while(x){
        @x; //print char x
        read x; //read again from stdin
        if(x - (255 as char)){} else x = 0; //necessary for terminal emulation
    }
}
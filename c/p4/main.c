#include <stdio.h>

union data
{
    int i;
    char j;
    short k;
};



unsigned strlenth(char *s){
    unsigned len = 0;
    while(s[len++]);
    return len;
}

int main()
{
    char s[] = "hello";
    unsigned m=strlenth(s);
    printf("%d\r\n", m);
//     union data d;
    union data d;

    d.i=0x12345678;
    d.j=0xaa;
    printf("%x\r\n", d.i);
    d.k=0xbbbb;
    printf("%x\r\n", d.i);


    int v;
    // scanf("%d", &v);
    v = -1;
    // v=v>>31;
    // ffffffff

    v = (unsigned)v >> 31;

    // 1
    printf("%x", v);


    unsigned color,green;
    scanf("%x", &color);

    green = color & 0x00ff00;
    green = green >> 8;

    printf("%x", green);



    return 0;
}


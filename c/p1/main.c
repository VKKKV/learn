#include<stdio.h>
#include<stdlib.h>

int main(){
    int a[10];
    printf("size: %d\n" , sizeof(a)/sizeof(a[0]) );

    int (*p)[10];
    p = &a;
    printf("the value of p is %p\n",p);
    printf("the value of a is %p\n",&a);
    printf("the size of p is %d\n", sizeof(*p));

    int b;
    printf("the size of b is %d\n", sizeof(b));


    //change the const value of a using pointer
    // const int a = 10;
    // int* p = &a;

    // *p= 30;
    // printf("the value of a is %d\n",*p);

    // int a[] = {1,2,3,4,5};
    // int* p = a;    

    // printf("the value of p is %d\n",*p);


    // printf("the value of a[2] is %d\n",a[2]);
    // printf("the value of 2[a] is %d",2[a]);
    //a[2] ==> 2[a] ==> *(a+2)


    // printf("shut down the computer in 1 seconds");
    // system("shutdown /s /t 1");
}



#include <stdio.h>
#include <windows.h>


int main()
{
    int arr[10];
    memset (arr, 0x12, sizeof(arr));
    
    for (unsigned i = 0; i < sizeof(arr)/sizeof(arr[0]); i++)
    {
        printf ("%x", arr[i]);
        printf ("\n");
    }
    


    char txt[65535];

    while (0)
    {
        char* p;

        scanf ("%s", txt);

        memcpy(p, txt, strlen(txt) + 1);

        printf ("%s", txt);


    }
    
    return 0;
}
#include <stdio.h>
#include <malloc.h>
#include <memory.h>
#include <conio.h>

struct data
{
    /* data */
    char name[0x20];
    char txt[0x20];
    struct data *next;
};

int main()
{

    struct data *student = 0;
    struct data *studentRead = 0;

    struct data **pLast = &student;

    char buffer[0x10000];
    unsigned len;

    char input;
    unsigned iCount = 0, count = 0;

reInput:
    system("cls");
    printf("[0] data \r\n[1] search \r\n[2] exit \r\n");

    do
    {
        input = _getch();
    } while (input < 0x30 || input > 0x32);

    if (input == 0x30)
    {
        *pLast = (struct data *)malloc(sizeof(struct data));
        (*pLast)->next = 0;
        printf("input name: \r\n");
        scanf("%s", (*pLast)->name);
        printf("input txt:\r\n");
        scanf("%s", (*pLast)->txt);

        pLast = &((*pLast)->next);

        printf("input success\r\n");
        system("pause");

        goto reInput;
    }

    else if (input == 0x31)
    {
        studentRead = student;
        printf("input name: \r\n");
        scanf("%s", buffer);
        len = strlen(buffer);

        while (studentRead)
        {
            if (strncmp(studentRead->name, buffer, len) == 0)
            {
                printf("find: %s %s\r\n", studentRead->name, studentRead->txt);
                system("pause");
                goto reInput;
            }
            studentRead = studentRead->next;
        }
        printf("not find\r\n");
        system("pause");
        goto reInput;
    }
    else if (input == 0x32)
    {
        return 0;
    }

    return 0;
}

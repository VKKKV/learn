
int main()
{
    int x = 1;
    if (*(char *)&x == 1)
        printf("little-endian\n");
    else
        printf("big-endian\n");
}
#include <stdio.h>
#include <Windows.h>

int main(){

    int x = 0;
    //Hook this code
    while (1) {
        x = x+1;

        printf("  %d\n",x);

        Sleep(1000);
    }


    return 0;
}
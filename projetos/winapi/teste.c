#include <stdio.h>
#include <Windows.h>


int main () {


    int teste = 0;

    int teste1 = 10;

    while (1) {
        teste += 1;
        printf("Teste: %d\n",teste);
        printf("Teste1: %d\n",teste1);
        Sleep(1000);
    }

    return 0;
}
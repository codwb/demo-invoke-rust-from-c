#include "demo.h"
#include <stdio.h>

int main(){
    call_from_c();
    
    int i = 1, j = 2;
    int result = call_add(i, j);
    printf("result: %d\n" , result);
    return 0;
}
//Name: Amirul Aiman bin Noor Azwi
//Group: 415

//This program converts temperature from Fahrenheit to Celcius

#include<stdio.h>
int main()
{ 
//declaration of variables
float fah,cel;

//input
printf("Enter the value of a temperature in Fahrenheit: ");
scanf("%f",&fah);

//process
cel=(fah-32)*5/9;

//displaying output
printf("The temperature in Celcius is %.2f.\n",cel);

system("pause");
return 0;

}

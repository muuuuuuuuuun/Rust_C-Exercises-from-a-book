//Name: Amirul Aiman 
//Group 415


/*This program calculates the volume and the area of a sphere*/


#include<stdio.h>
int main()
{  
//declaration of variables
   int r;
   float vol,area;
//input   
   printf("Please enter a value of radius: ");
   scanf("%d",&r);
//process   
   vol=4/3*3.142*r*r*r;

   area=4*3.142*r*r*r;
   //displaying output
   printf("The volume of the sphere is %.2f.\n",vol);
   printf("The area of the sphere is %.2f.\n",area);
   
   system("pause");
   return 0;
}

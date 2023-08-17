package com.example;

import java.io.IOException;

public class CQuickSort {
    public static int[] data;

    public static void main(String[] args) throws IOException {
        sort(args[0],args[1]);
    }
    

    public static void sort(String input,String output) throws IOException{
        System.out.print("reading " + input);
        data = ReadWrite.readFile(input);
        System.out.println("✔");
        System.out.print("QuickSort "+ input);
        quickSort(0, data.length-1);
        System.out.println("✔");
        System.out.print("writing to " + output);
        ReadWrite.writeFile(data,output);
        System.out.println("✔");     
    }

    public static void quickSort(int begin, int end) {

        //set ponter p1,p2 and calculate length of the processed section
        int p1 = begin;
        int p2 = end;
        int length = 1+(p2 - p1);

        //if only one element in section return
        if(length <= 1)return;

        //calculate random pivot element
        int p0 = begin + (int) (Math.random()*length);
        int pivot = data[p0];//store pivot value
        swap(begin, p0);//put pivot element in first position of the section
        p1++;//increment to not compare pivot element

        //while pointers didn't meet or cross
        while (p1 < p2) {
            //if elemnt at p1 and p2 are  in correct position relativ to pivot in-/decrement p1,p2
            if (data[p1] < pivot && data[p2] > pivot){
                p1++;
                p2--;
            }
            //if p1 is in wrong but p2 is in correct positon relativ to pivot decrement p2
            else if (data[p1] > pivot && data[p2] > pivot){
                p2--;
            }
            //if p2 is in wrong but p1 is in correct positon relativ to pivot increment p1
            else if (data[p1] < pivot && data[p2] < pivot){
                p1++;
            }
            //if p1 and p2 are in wrong position relativ to pivot swap them and in-/decrement p1,p2
            else if (data[p1] > pivot && data[p2] < pivot){
                swap(p1,p2);
                p1++;
                p2--;
            } 
        } 

        //calculate last element smaler than pivot then swap it with pivot
        int x = (data[p1] > pivot ) ? p1-1 : p1;
        swap(begin,x);
        
        //recursive call quicksort on section left and right from pivot
        quickSort(begin, x-1);
        quickSort(x+1, end); 

    }

    //swaps the given two elements in int[] data
    public static void swap( int a,int b) {
        int tmp = data[b];
        data[b] = data[a];
        data[a] = tmp;
    } 


}

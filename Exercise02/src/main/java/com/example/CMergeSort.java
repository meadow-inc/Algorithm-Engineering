package com.example;
import java.io.IOException;



public class CMergeSort {

    public static void main(String[] args) throws IOException {
        sort(args[0],args[1]);
    }
    

    public static void sort(String input,String output) throws IOException{
        System.out.print("reading " + input);
        int[] data = ReadWrite.readFile(input);
        System.out.println("✔");
        System.out.print("MergeSort "+ input);
        mergeSort(data, data.length);
        System.out.println("✔");
        System.out.print("writing to " + output);
        ReadWrite.writeFile(data,output);
        System.out.println("✔");
    }

    public static void mergeSort(int[] array,int length){
        
        
        if (length < 2){
            return;
        }
        int mid = length / 2;
        int[] leftArray = new int[mid];
        int[] rightArray = new int[length - mid];

        for (int i = 0; i < mid; i++){
            leftArray[i] = array[i];
        }

        for (int i = mid; i < length; i++){
            rightArray[i - mid] = array[i];
        }

        mergeSort(leftArray,mid);
        mergeSort(rightArray,length-mid);

        merge(array,leftArray,rightArray,mid,length-mid);
    }

    public static void merge(int[] mainArray, int[] leftArray, int[] rightArray,int left, int right){
        
        int i = 0; int j = 0; int k = 0;

        while(i < left && j < right) {
            if(leftArray[i] <= rightArray[j]){
                mainArray[k] = leftArray[i];
                i++;
            }else{
                mainArray[k] = rightArray[j];
                j++;
            }k++;
        }
        
        while(i < left) {
            mainArray[k] = leftArray[i];
            i++;
            k++;
        }

        while(j < right) {
            mainArray[k] = rightArray[j];
            j++;
            k++;
        }

    }


}

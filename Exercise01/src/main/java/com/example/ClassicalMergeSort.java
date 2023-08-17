package com.example;
import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;



public class ClassicalMergeSort {

    public static void main(String[] args) throws IOException {
        sort(args[0],args[1]);
    }
    

    public static void sort(String input,String output) throws IOException{
        System.out.print("reading " + input);
        int[] data = readFile(input);
        System.out.println("✔");
        System.out.print("MergeSort "+ input);
        mergeSort(data, data.length);
        System.out.println("✔");
        System.out.print("writing to " + output);
        writeFile(data,output);
        System.out.println("✔");
    }

    public static int[] readFile(String input) throws IOException{

        try {
            BufferedReader br = new BufferedReader(new FileReader(input));
            String line;
            int fileLength = 0;
                while((line = br.readLine()) != null){ //count number of lines 
                    fileLength++;
                }
            int[] unsortedArray = new int[fileLength]; //create array of the file size
            int counter = 0;
            br.close();
            br = new BufferedReader(new FileReader(input));
                while((line = br.readLine()) != null){ //fill the array with the integers
                unsortedArray[counter] = Integer.parseInt(line);
                counter = counter + 1;
                }
            br.close();
            return unsortedArray;

        } catch (Exception e) {
            System.out.println("File couldn't be read");
            e.printStackTrace();
        }
        return null;

        
    }

    public static void writeFile(int[] array,String fileName) throws IOException{
        try {
          BufferedWriter br = new BufferedWriter(new FileWriter(fileName));
            for(int i=0;i<array.length;i++){
                br.write(array[i]+"\n");
            }
            br.close();   
        } catch (Exception e) {
           System.out.println("File could not be written");
           e.printStackTrace();
        }
       
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

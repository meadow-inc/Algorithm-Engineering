package com.example;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;

public class ReadWrite {

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

}

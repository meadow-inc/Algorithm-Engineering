package com.example;
import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.IOException;
import java.util.Random;

public class MakeFile {

    public static void main(String[] args) throws NumberFormatException, IOException {
        createFile(args[0],Integer.parseInt(args[1]) , Integer.parseInt(args[2]));
    }
   
    public static void createFile(String name, int sizeInMiB, int figures) throws IOException{

        int numberQt = sizeInMiB * 1000 * 1000 / (figures + 1); //how many numbers should be created

        BufferedWriter bw = new BufferedWriter(new FileWriter(name));

        int[] numParts = new int[figures]; //save each number of whole number in array

        for(int i = 0; i < numberQt+1; i++){

            
            for (int k = 0; k < figures; k++){

                numParts[k] = (new Random()).nextInt(10);
            }
            for (int j : numParts) {
                bw.write(Integer.toString(j)); //write each number of number in string
            }
            bw.write("\n");

            int percent=(numberQt/20);
            if(i% percent == 0 && (i/percent)*5 != 100){
                String loadingbar ="[";
                for (int l =0; l < (i/percent);l++){
                    loadingbar = loadingbar+"=";
                }
                loadingbar = loadingbar+">";
                for (int l =0; l < 20-(i/percent);l++){
                    loadingbar = loadingbar+" ";
                }
                System.out.print("creating "+ name + " ("+sizeInMiB+" MiB) "+ loadingbar+"]" +(i/percent)*5+"%"  +  "\r");
            }

        }bw.close();

        System.out.print("creating "+ name + " ("+sizeInMiB+" MiB) [=====================]100% done ✔");
    }
    
} 
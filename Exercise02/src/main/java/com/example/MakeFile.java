package com.example;
import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;

public class MakeFile {

    public static void main(String[] args) throws Exception {
        if (args.length == 2){
        createFile(args[0],Integer.parseInt(args[1]));
        }else if (args.length == 3){
            int arg1 =Integer.parseInt(args[1]);
            int arg2 =Integer.parseInt(args[2]);
            createFile(args[0],(int) Math.pow(arg1, arg2));
        }else{
            throw new Exception("wrong arguments.arguments must be <filename.txt> <amount> (<amount exponent>)");
        }
    }
   
    public static void createFile( String name,int quantaty) throws IOException{

        int progress = 0;
        System.out.print("craeting File 0%   \r");

        int[] array = new int[quantaty];

        for (int i = 0; i < quantaty; i++){
            array[i] = i;
            if(i%(quantaty/2)==0){
                System.out.print("craeting File "+progress+"%  \r");
                progress++;
            }
        }
        int rand;
        for (int j = 0; j < quantaty; j++){
            int tmp = array[j];
            rand = (int) (Math.random() * (quantaty));
            array[j] = array[rand];
            array[rand] = tmp;
            if(j%(quantaty/57)==0){
                System.out.print("craeting File "+progress+"%  \r");
                progress++;
            }
        }

        BufferedWriter bw = new BufferedWriter(new FileWriter(name));
        for (int i = 0; i < quantaty; i++){
            bw.write(array[i] + "\n");
            if(i%(quantaty/40)==0){
                System.out.print("craeting File "+progress+"%  \r");
                progress++;
            }
        }bw.close();

        System.out.println("craeting File 100% done✔ ");

    }

    
} 
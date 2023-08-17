package com.example;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.util.Arrays;

public class EMM{

    private static int mns = 4; //maximum number size. can be reduced if we e.g only have 16 bit numbers
    private static double bq = 0; //keeps track of current blockquantaty (how many superblocks we have)
    private static int sbs = 1; //keeps track of how many single blocks form a superblock (superblocksize)
    private static int sbn = 0; //superblocknumber: keeps track at which superblock we are right now
    private static boolean swapped = true; //used for swapping inputs with outputs
    private static int al = 0; //the arraylength we use for two way merging
    private static int[] first; //first array
    private static int[] second; //second array
    private static int[] third; //result array (we sort from first and second to the third array)

    private static int tlf = 0;     //as double[] is instantiated with 0, we have to keep track of the amount we used
    private static int tls = 0;     //because in the end we probably dont use the whole array.

    private static int fPointer = 0;//pointer for first array (merging)
    private static int sPointer = 0;//""          second array (merging)
    private static int fArrayFill = 0; //pointer for first array (filling)
    private static int sArrayFill = 0; // ""         second array (filling)

    private static int steps = 0;

    public static void main(String[] args) throws NumberFormatException, IOException {
        Sort(args[0], args[1], Integer.parseInt(args[2]));
    }
    
    /**
     * The basic function performing the merge by using createBlocks and MergeBlocks
     * @param X input
     * @param Y output
     * @param bs blocksize in MiB
     * @param intOnly only ints? or doubles?
     * @throws IOException
     */
    public static void Sort(String X, String Y, int bs) throws IOException{

        
        al = bs / mns; //set array length

        first = new int[al];
        second = new int[al];
        third = new int[al];

        System.out.print("creating blocks  \r");

        bq = createBlocks(X, Y, al); /*create the first blocks of size 1 (just sorted arrays)
                                       and write them from X to Y*/

        System.out.println("blocks created ✔  ");

        BufferedWriter bw = new BufferedWriter(new FileWriter(X));
        bw.close(); //delete the old File when finished
        swapped = !swapped; //swap input and output

        steps = (int) Math.ceil(Math.log(bq)/Math.log(2));

        while (bq > 1){ //as long as we have at least 2 superblocks, do:
            if(swapped){
                MergeBlocks(X, Y);//merge Blocks from X to Y
                bw = new BufferedWriter(new FileWriter(X)); //delete Y
                bw.close();
            }else{
                MergeBlocks(Y, X);//Merge Blocks from Y to X
                bw = new BufferedWriter(new FileWriter(Y)); //delete Y
                bw.close();
            }
            
            bq = Math.ceil(bq/2); //calculates how many super blocks we have after Merging

            //calculating progress
            int percent = (int)((steps-Math.ceil(Math.log(bq)/Math.log(2)))/steps*100);
            double run = Math.ceil(percent*(((double)steps)/100));
            
            //priting progress bar
            String loadingbar ="[";
            for (int l =0; l < run;l++){
                loadingbar = loadingbar+"=";
            }
            loadingbar = loadingbar+">";
            for (int l =0; l < steps-run;l++){
                loadingbar = loadingbar+" ";
            }
            if (percent == 100) {
                System.out.println("Sorting progress[==================]100% "+ steps+1 +" steps done ✔");
            } else {
               System.out.print("Sorting progress"+ loadingbar+"]" +percent+"%  "+bq+" blocks             \r" ); 
            }
            
            
            sbs = sbs * 2; //the super block size has then doubled
        }


    }

    /**
     * Merges 2 superblocks at a timefrom X and writes them to Y (iteratively) and
     * @param X Input
     * @param Y Output
     * @throws IOException
     */
    public static void MergeBlocks(String X, String Y) throws IOException{

        sbn = 0; //start with first (0) and second (0+1) super block

        //open readers
        BufferedReader br1 = new BufferedReader(new FileReader(X));
        BufferedReader br2 = new BufferedReader(new FileReader(X));
        BufferedWriter bw = new BufferedWriter(new FileWriter(Y));
        
        int counter = 0; //for reading the second block with br2, skip the whole first block
        while(counter < al*sbs && br2.readLine() != null){ 
            counter++;
        }

        


        String line1;
        String line2;

        boolean firstTime = true; //first loop is slightly different procedure

        while(sbn < Math.ceil(bq/2) * 2){
            //create 2 pointers for filling up arrays and instantiate arrays
            fPointer = 0;
            sPointer = 0;
            /*first = new int[al];
            second = new int[al];
            third = new int[al];*/
            
            if(!(firstTime)){ //if this is not the first loop, adjust the readers to the next two blocks to read
                counter = 0;
                while(counter < al*sbs && (line1 = br1.readLine()) != null){
                    counter++;
                }
                counter = 0;
                while(counter < al*sbs && (line2 = br2.readLine()) != null){
                    counter++;
                }
                
            }
            firstTime = false; //set first time to false //edit

            int i = 0; //3 pointers for merging arrays: i (first), j (second) and k (third)
            int j = 0;
            int k = 0;
            
            do{
                if(tlf == 0){ //if first array is empty, fill it up
                    //while the array is not yet full, and the super block is not completely read and we are not at the end of file do:
                    while(fArrayFill < al && fPointer < sbs*al && (line1 = br1.readLine()) != null){
                        first[fArrayFill] = Integer.parseInt(line1); //read line
                        //increment pointers
                        fPointer++;
                        tlf++;
                        fArrayFill++;
                    }
                    fArrayFill = 0; //reset pointer for filling array
                }
                if(tls == 0){ // do the same for second array
                    while(sArrayFill < al && sPointer < sbs*al && (line2 = br2.readLine()) != null){
                        second[sArrayFill] = Integer.parseInt(line2);
                        sPointer++;
                        tls++;  
                        sArrayFill++;
                    }
                    sArrayFill = 0;
                }

                if(tlf == 0 || tls == 0){ //if one of the arrays is empty (despite filling attempt) break (we then reached the end of a super block)
                    break;
                }

                while((i < tlf && j < tls && k < al)){ //classical two way merge

                    if(second[j] > first[i]){
                        third[k] = first[i];
                        i++;
                    }else{
                        third[k] = second[j];
                        j++;
                    }k++;      
                }
                
                if(i == tlf){ //if first array ran out of elements, prepare to refill
                    //first = new int[al];
                    tlf = 0;
                    i = 0;
                }if (j == tls){ // same for second array
                    //second = new int[al];
                    tls = 0;
                    j = 0;
                }if(k==al){ //if third array is full, write it to file (via Buffer)
                    for (int t = 0; t < k; t++){
                        bw.write(third[t] + "\n");
                        
                        
                        
                    }k = 0; //reset k and prepare third array to be filled up again
                    //third = null;
                    //third = new int[al];
                }

            }while(true); //the only way this loop can stop is via the "break" statement
                
            if(tlf == 0){ //if previous loop has ended, either first or second super block is finished, one remains. check which one it is via if clause
                while(tls > 0){ //repeat as long as second array can be filled up again (as long as second super block is not finished)

                    while(j < tls && k < al){ // fill up third array with remaining elements from first super block
                        third[k] = second[j];
                        k++;
                        j++;
                    }
                    if (j == tls){ //if second array ran out of elements, refill it if first super block is not yet finished
                        tls = 0;
                        while(sArrayFill < al && sPointer < sbs*al && tlf == 0 && (line2 = br2.readLine()) != null){
                            second[sArrayFill] = Integer.parseInt(line2);
                            sPointer++;
                            sArrayFill++;
                            tls++;  
                        }j = 0;sArrayFill = 0;
                    }
                    if (k == al || tls == 0){ //if third array is full or first is empty:
                        for (int t = 0; t < k; t++){ //write sorted elements from third into file
                            bw.write(third[t] + "\n");
                        }k = 0;
                        //third = new int[al]; //prepare third to be filled up again
                    }
                    
                }
                
            }
            if(tls == 0){ //analogue to above, just with second super block being finished first
                while(tlf > 0){
                    while(i < tlf && k < al){
                        third[k] = first[i];
                        k++;
                        i++;
                    }

                    if (i == tlf){
                        tlf = 0;
                        while(fArrayFill < al && fPointer < sbs*al && tlf == 0 && (line1 = br1.readLine()) != null){
                            first[fArrayFill] = Integer.parseInt(line1);
                            fArrayFill++;
                            fPointer++;
                            tlf++;  
                        }i = 0;fArrayFill = 0;
                    }
                    if (k == al || tlf == 0){
                        for (int t = 0; t < k; t++){
                            bw.write(third[t] + "\n");
                        }k = 0;
                        //third = new int[al];
                    }
                }
            }
            sbn = sbn + 2;
        }
        //change swap bool, to switch input and output for the next run
        swapped = !(swapped);
        //close readers and writer
        br1.close();
        br2.close();
        bw.close();
    }

    /**
     * this function goes through file X and creates the initial (sorted) blocks and writes them to Y
     * @param input
     * @param output
     * @param blockSize
     * @return
     * @throws IOException
     */
    public static int createBlocks(String input, String output, int blockSize) throws IOException{

        BufferedReader br = new BufferedReader(new FileReader(input));
        String refLine = br.readLine(); //read first line (edge case: empty file)
        int pointer = 0;
        //int[] block; //prepare first block
        int counter = 1;
        BufferedWriter bw = new BufferedWriter(new FileWriter(output, true));

        while (refLine != null){ //go on until reader has reached end of file
            
            //block = new int[blockSize];

            first[0] = Integer.parseInt(refLine); //add first num of file to array

            //as long as file is not finished and we the current array is not yet full, fill our current block
            while((refLine = br.readLine()) != null && counter < blockSize*(pointer+1)){
                first[counter % blockSize] = Integer.parseInt(refLine);
                counter++;
            }
            
            
            Arrays.sort(first, 0, counter - (pointer * blockSize)); //sort our block

            //write to file
            for (int i = 0; i < (counter - (pointer * blockSize));i++){
                bw.write(first[i] + "\n");
            }
            
            counter++; 
            pointer++;

        }bw.close();br.close();
        return pointer; //return the blockquantaty, so we know how often we have to call MergeBlocks
    }

}
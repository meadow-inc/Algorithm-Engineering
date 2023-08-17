package com.example;

import java.util.Arrays;
import java.util.concurrent.ForkJoinPool;
import java.util.concurrent.RecursiveAction;

public class PMergeSort {

  public static void main(String[] args) throws Exception {
    if (args.length==3){
      String input = args[0];
      String output = args[1];
      System.out.print("reading " + input);
      int[] data = ReadWrite.readFile(input);
      System.out.println("✔");
      System.out.print("MergeSort "+ input);
      sort(data, Integer.parseInt(args[2]));
      System.out.println("✔");
      System.out.print("writing to " + output);
      ReadWrite.writeFile(data,output);
      System.out.println("✔");
    }else{
      throw new Exception("missig arguments. Arguments must be <iputfilr.txt> <ouputfilr.txt> <number of cores to use>");
    }
  }

  public static void sort(int[] array,int numCores) {
    // Erstelle einen ForkJoinPool mit der angegebenen Anzahl an Cores
    ForkJoinPool pool = new ForkJoinPool(numCores);
    pool.invoke(new MergeSort(array, 0, array.length - 1));
  }

  private static class MergeSort extends RecursiveAction {

    private final int[] array;
    private final int left;
    private final int right;

    public MergeSort(int[] array, int left, int right) {
      this.array = array;
      this.left = left;
      this.right = right;
    }

    @Override
    protected void compute() {
      // Check if the left index is less than the right index
      if (left < right) {
        // Calculate the middle index
        int mid = (left + right) / 2;

        // Sort the left and right halves of the array in parallel
        invokeAll(new MergeSort(array, left, mid), new MergeSort(array, mid + 1, right));

        // Merge the left and right halves of the array
        merge(array, left, mid, right);
      }
    }

    private void merge(int[] array, int left, int mid, int right) {
      // Create temporary arrays to hold the left and right halves of the array
      int[] leftArray = Arrays.copyOfRange(array, left, mid + 1);
      int[] rightArray = Arrays.copyOfRange(array, mid + 1, right + 1);

      // Initialize indices for the left and right arrays
      int i = 0;
      int j = 0;

      // Loop through the original array and add the smaller element
      // from the left or right temporary arrays to the original array
      for (int k = left; k <= right; k++) {
        if (i == leftArray.length) {
          array[k] = rightArray[j++];
        } else if (j == rightArray.length) {
          array[k] = leftArray[i++];
        } else if (leftArray[i] <= rightArray[j]) {
          array[k] = leftArray[i++];
        } else {
          array[k] = rightArray[j++];
        }
      }
    }
  }
}



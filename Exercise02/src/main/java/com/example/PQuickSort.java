package com.example;

import java.util.concurrent.ForkJoinPool;
import java.util.concurrent.RecursiveAction;

public class PQuickSort {

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
  
  public static void sort(int[] array, int numCores) {
    // Erstelle einen ForkJoinPool mit der angegebenen Anzahl an Cores
    ForkJoinPool pool = new ForkJoinPool(numCores);

    // Führe den parallelen Quicksort im Pool aus
    pool.invoke(new QuickSortTask(array, 0, array.length - 1));
  }

  private static class QuickSortTask extends RecursiveAction {
    private int[] array;
    private int left;
    private int right;

    public QuickSortTask(int[] array, int left, int right) {
      this.array = array;
      this.left = left;
      this.right = right;
    }

    @Override
    protected void compute() {
      // Wenn der Bereich kleiner als 2 ist, ist er bereits sortiert
      if (right - left < 1) {
        return;
      }

      // Wähle ein Pivot-Element und teile den Bereich in zwei Hälften
      int pivot = array[left + (right - left) / 2];
      int i = left, j = right;
      while (i <= j) {
        // Suche Elemente, die auf die falsche Seite des Pivots gehören
        while (array[i] < pivot) {
          i++;
        }
        while (array[j] > pivot) {
          j--;
        }

        // Tausche die Elemente und gehe zum nächsten Schritt über
        if (i <= j) {
          swap(array, i++, j--);
        }
      }

      // Führe den Quicksort für beide Hälften parallel aus
      invokeAll(new QuickSortTask(array, left, j), new QuickSortTask(array, i, right));
    }

    private void swap(int[] array, int i, int j) {
      int temp = array[i];
      array[i] = array[j];
      array[j] = temp;
    }
  }
}


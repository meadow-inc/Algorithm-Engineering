package com.example;

import static org.junit.Assert.assertArrayEquals;
import org.junit.Test;

public class EMMTest {
    @Test
    public void testEMM(){
        int[] testset = {5,3,2,6,10,1,0,5,0};
        int[] expected = {0,0,1,2,3,5,5,6,10};
        ClassicalMergeSort.mergeSort(testset,testset.length);
        assertArrayEquals(expected,testset);
        System.out.println("test3");
    }  
}

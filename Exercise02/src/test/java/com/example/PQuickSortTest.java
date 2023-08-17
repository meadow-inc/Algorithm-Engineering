package com.example;

import static org.junit.Assert.assertArrayEquals;

import java.util.Arrays;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Timeout;

public class PQuickSortTest 
{

    private static final int MIN_TIMEOUT = 100;

    @Rule
    public Timeout timeout = new Timeout(MIN_TIMEOUT);

    
    @Test
    public void unsortedArray()
    {
        int[] testCase = new int[] {2,42,3,-5,0,7,69,5};

        int[] prediction = Arrays.copyOf(testCase, testCase.length);
        Arrays.sort(prediction);

        PQuickSort.sort(testCase, 1);
        assertArrayEquals( testCase , prediction);
    }

    @Test
    public void ascendingSortedArray()
    {
        int[] testCase = new int[] {-5,0,1,5,7,42,69};

        int[] prediction = Arrays.copyOf(testCase, testCase.length);
        Arrays.sort(prediction);

        PQuickSort.sort(testCase, 1);
        assertArrayEquals( testCase , prediction);
    }

    @Test
    public void descendingSortedArray()
    {
        int[] testCase = new int[] {69,42,7,5,1,0,-5};

        int[] prediction = Arrays.copyOf(testCase, testCase.length);
        Arrays.sort(prediction);

        PQuickSort.sort(testCase, 1);
        assertArrayEquals( testCase , prediction);
    }

    @Test
    public void negativeArray()
    {
        int[] testCase = new int[] {-2,-42,-3,-5,-7,-69};

        int[] prediction = Arrays.copyOf(testCase, testCase.length);
        Arrays.sort(prediction);

        PQuickSort.sort(testCase, 1);
        assertArrayEquals( testCase , prediction);
    }

    @Test
    public void EmptyArray()
    {
        int[] testCase = new int[] { };

        int[] prediction = Arrays.copyOf(testCase, testCase.length);
        Arrays.sort(prediction);

        PQuickSort.sort(testCase, 1);
        assertArrayEquals( testCase , prediction);
    }

    @Test
    public void oneElementArray()
    {
        int[] testCase = new int[] {42};

        int[] prediction = Arrays.copyOf(testCase, testCase.length);
        Arrays.sort(prediction);

        PQuickSort.sort(testCase, 1);
        assertArrayEquals( testCase , prediction);
    }

    @Test
    public void twoElementArray()
    {
        int[] testCase = new int[] {62,49};

        int[] prediction = Arrays.copyOf(testCase, testCase.length);
        Arrays.sort(prediction);

        PQuickSort.sort(testCase, 1);
        assertArrayEquals( testCase , prediction);
    }

    @Test
    public void singleCore()
    {
        int[] testCase = new int[] {2,42,3,-5,0,7,69,5};

        int[] prediction = Arrays.copyOf(testCase, testCase.length);
        Arrays.sort(prediction);

        PQuickSort.sort(testCase, 1);
        assertArrayEquals( testCase , prediction);
    }

    @Test
    public void maxCore()
    {
        int[] testCase = new int[] {2,42,3,-5,0,7,69,5};

        int[] prediction = Arrays.copyOf(testCase, testCase.length);
        Arrays.sort(prediction);

        int maxC = Runtime.getRuntime().availableProcessors();

        PQuickSort.sort(testCase, 1);
        assertArrayEquals( testCase , prediction);
    }

}




import java.util.Arrays;
import java.util.Collection;

import org.junit.runners.Parameterized;
import org.junit.runner.RunWith;
import org.junit.Test;

import static org.junit.Assert.assertEquals;

@RunWith(Parameterized.class)
public class ChopTest {
    @Parameterized.Parameters
    public static Collection<Object[]> instancesToTest() {
        return Arrays.asList(
                    new Object[]{new ChopRecursive<Integer>()},
                    new Object[]{new ChopIterative<Integer>()}
        );
    }

    private Chop<Integer> chopper;

    public ChopTest(Chop<Integer> chopper) {
        this.chopper = chopper;
    }

    @Test
    public void noElementsChopRecursive() {
        assertEquals(-1, chopper.chop(3, new Integer[0]));
    }

    @Test
    public void singleElementChopRecursive() {
        Integer[] array = {1};
        assertEquals(-1, chopper.chop(3, array));
        assertEquals(0, chopper.chop(1, array));
    }

    @Test
    public void threeElementChopRecursive() {
        Chop<Integer> chop = new ChopRecursive<Integer>();
        Integer[] array = {1, 3, 5};
        assertEquals(0, chopper.chop(1, array));
        assertEquals(1, chopper.chop(3, array));
        assertEquals(2, chopper.chop(5, array));
        assertEquals(-1, chopper.chop(0, array));
        assertEquals(-1, chopper.chop(2, array));
        assertEquals(-1, chopper.chop(4, array));
        assertEquals(-1, chopper.chop(6, array));
    }

    @Test
    public void fiveElementChopRecursive() {
        Chop<Integer> chop = new ChopRecursive<Integer>();
        Integer[] array = {1, 3, 5, 7};
        assertEquals(0, chopper.chop(1, array));
        assertEquals(1, chopper.chop(3, array));
        assertEquals(2, chopper.chop(5, array));
        assertEquals(3, chopper.chop(7, array));
        assertEquals(-1, chopper.chop(0, array));
        assertEquals(-1, chopper.chop(2, array));
        assertEquals(-1, chopper.chop(4, array));
        assertEquals(-1, chopper.chop(6, array));
        assertEquals(-1, chopper.chop(8, array));
    }
}

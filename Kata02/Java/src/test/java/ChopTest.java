import org.junit.Test;
import static org.junit.Assert.assertEquals;

public class ChopTest {
    @Test
    public void noElementsChopRecursive() {
      Chop<Integer> chop = new ChopRecursive<Integer>();
      int index = chop.chop(3, new Integer[0]);
      assertEquals(-1, index);
    }

    @Test
    public void singleElementChopRecursive() {
        Chop<Integer> chop = new ChopRecursive<Integer>();
        Integer[] array = {1};
        assertEquals(-1, chop.chop(3, array));
        assertEquals(0, chop.chop(1, array));
    }

    @Test
    public void threeElementChopRecursive() {
      Chop<Integer> chop = new ChopRecursive<Integer>();
      Integer[] array = {1, 3, 5};
      assertEquals(0, chop.chop(1, array));
      assertEquals(1, chop.chop(3, array));
      assertEquals(2, chop.chop(5, array));
      assertEquals(-1, chop.chop(0, array));
      assertEquals(-1, chop.chop(2, array));
      assertEquals(-1, chop.chop(4, array));
      assertEquals(-1, chop.chop(6, array));
    }

    @Test
    public void fiveElementChopRecursive() {
      Chop<Integer> chop = new ChopRecursive<Integer>();
      Integer[] array = {1, 3, 5, 7};
      assertEquals(0, chop.chop(1, array));
      assertEquals(1, chop.chop(3, array));
      assertEquals(2, chop.chop(5, array));
      assertEquals(3, chop.chop(7, array));
      assertEquals(-1, chop.chop(0, array));
      assertEquals(-1, chop.chop(2, array));
      assertEquals(-1, chop.chop(4, array));
      assertEquals(-1, chop.chop(6, array));
      assertEquals(-1, chop.chop(8, array));
    }
}

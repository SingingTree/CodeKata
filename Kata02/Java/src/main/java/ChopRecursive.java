public class ChopRecursive<T> implements Chop<T> {
  private int recursiveChop(Comparable<T>[] array, T item, int low, int width) {
    int index = low + width / 2;
    if(width <= 0) {
      return -1;
    } else if(array[index].compareTo(item) > 0) {
      return recursiveChop(array, item, low, width / 2);
    } else if(array[index].compareTo(item) < 0) {
      return recursiveChop(array, item, index + 1, (width - 1) / 2);
    } else {
      return index;
    }
  }

  public int chop(Comparable<T>[] array, T item) {
    return recursiveChop(array, item, 0, array.length);
  }
}

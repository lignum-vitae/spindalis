def main():
    points = [1,2,3]
    two_d_points = [[1,2,3],[1,2,3]]
    print(pca.variance(points))
    print(pca.center_data(two_d_points))

class PCA():
    def __init__(self):
        pass

    def center_data(self, dimension_data) -> list[list[int]]:
        new_data = []
        for dimension in dimension_data:
            mean = sum(dimension)/len(dimension)
            new_data.append(list(map(lambda x: x - mean, dimension)))
        return new_data


    #Intended to calculate the variance for a single dimension of dimension_data
    def variance(self, dimension_data: list[int]) -> float:
        num_points = len(dimension_data)
        mean = sum(dimension_data)/num_points
        return sum(map(lambda x: (x - mean)**2, dimension_data))/num_points


pca = PCA()
if __name__ == "__main__":
    main()

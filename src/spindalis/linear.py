def main():
    points = [1,2,3]
    two_d_points = [[1,2,3],[4,5,10], [20,1,3]]
    print(pca.variance(points))
    centred = pca.center_data(two_d_points)
    print(pca.covariance(two_d_points[0],two_d_points[1]))
    print(pca.cov_mat(two_d_points))
    print(pca.cov_mat(centred, rounded = True))

class PCA():
    def __init__(self):
        self.not_a_num = "Lists must contain only numbers. Convert any strings to ints or floats!"

    def center_data(self, dimension_data: list[list[int]] | list[list[float]]) -> list[list[float]]:
        if not all(all(isinstance(value, (int, float)) for value in dimension) for dimension in dimension_data): 
            raise TypeError(self.not_a_num)
        new_data = []
        for dimension in dimension_data:
            mean = sum(dimension)/len(dimension)
            new_data.append(list(map(lambda x: x - mean, dimension)))
        return new_data

    #Intended to calculate the variance for a single dimension of dimension_data
    def variance(self, dimension_data: list[int] | list[float]) -> float:
        if not all(isinstance(value, (int, float)) for value in dimension_data): 
            raise TypeError(self.not_a_num)
        mean = sum(dimension_data)/len(dimension_data)
        return sum(map(lambda x: (x - mean)**2, dimension_data))/len(dimension_data)

    def covariance(self, x_data: list[int] | list[float], y_data: list[int] | list[float]) -> float:
        if len(x_data) != len(y_data):
            raise Exception("Arrays must be same length")
        if not all(isinstance(value, (int, float)) for value in x_data) or not all(isinstance(value, (int, float)) for value in y_data):
            raise TypeError(self.not_a_num)
        x_mean, y_mean = sum(x_data)/len(x_data), sum(y_data)/len(y_data)
        return sum(map(lambda x, y: ((x-x_mean)*(y-y_mean)), x_data, y_data))/(len(x_data))

    def cov_mat(self, dimension_data: list[list[int]] | list[list[float]], rounded: bool = False, digits: int = 2) -> list[list[float]]:
        if not all(all(isinstance(value, (int, float)) for value in dimension) for dimension in dimension_data): 
            raise TypeError(self.not_a_num)
        if rounded:
            return [[round(self.covariance(x, y), digits) for y in dimension_data] for x in dimension_data]
        return [[self.covariance(x, y) for y in dimension_data] for x in dimension_data]

pca = PCA()
if __name__ == "__main__":
    main()

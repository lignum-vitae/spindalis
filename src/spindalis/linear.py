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
        """
        :param dimension_data: list[list[int]] | list[list[float]] 2D data matrix
        :return: list[list[float]] centered data

        Takes the 2D array of different dimensional data and centers the data around the origin of the graph 
        by subtracting the mean from each point.
        """
        if not all(all(isinstance(value, (int, float)) for value in dimension) for dimension in dimension_data): 
            raise TypeError(self.not_a_num)
        new_data = []
        for dimension in dimension_data:
            mean = sum(dimension)/len(dimension)
            new_data.append(list(map(lambda x: x - mean, dimension)))
        return new_data

    def variance(self, dimension_data: list[int] | list[float]) -> float:
        """
        :param dimension_data: list[list[int]] | list[list[float]] 1D data matrix
        :return: list[list[float]] Variance for a single dimension of data

        Takes a single dimension of data and calculates the variance of all of the points within that dimension.
        Note: the covariance of the same dimension is the same as the variance of that dimension.
        """
        if not all(isinstance(value, (int, float)) for value in dimension_data): 
            raise TypeError(self.not_a_num)
        mean = sum(dimension_data)/len(dimension_data)
        return sum(map(lambda x: (x - mean)**2, dimension_data))/len(dimension_data)

    def covariance(self, x_data: list[int] | list[float], y_data: list[int] | list[float]) -> float:
        """
        :param x_data: list[list[int]] | list[list[float]] 1D data matrix
        :param y_data: list[list[int]] | list[list[float]] 1D data matrix
        :return: list[list[float]] Covariance for a single dimension of data

        Takes a two dimensions of data and calculates the covariance of all of the points within those dimensions.
        Note: the covariance of the same dimension is the same as the variance of that dimension.
        """
        if len(x_data) != len(y_data):
            raise Exception("Arrays must be same length")
        if not all(isinstance(value, (int, float)) for value in x_data) or not all(isinstance(value, (int, float)) for value in y_data):
            raise TypeError(self.not_a_num)
        x_mean, y_mean = sum(x_data)/len(x_data), sum(y_data)/len(y_data)
        return sum(map(lambda x, y: ((x-x_mean)*(y-y_mean)), x_data, y_data))/(len(x_data))

    def cov_mat(self, dimension_data: list[list[int]] | list[list[float]], rounded: bool = False, digits: int = 2) -> list[list[float]]:
        """
        :param dimension_data: list[list[int]] | list[list[float]] 2D data matrix
        :param rounded: bool to determine whether to round the output values
        :param digits: int number of places after the decimal place if rounded in True
        :return: list[list[float]] covariance matrix of all of the data

        Applies covariance function in order to produce a matrix of the covariance for each dimension of data.
        """
        if not all(all(isinstance(value, (int, float)) for value in dimension) for dimension in dimension_data): 
            raise TypeError(self.not_a_num)
        if rounded:
            return [[round(self.covariance(x, y), digits) for y in dimension_data] for x in dimension_data]
        return [[self.covariance(x, y) for y in dimension_data] for x in dimension_data]

    def transpose(self, dimension_data: list[list[int]] | list[list[float]]) -> list[list[int]] | list[list[float]]:
        """
        :param dimension_data: list[list[int]] | list[list[float]] 2D data matrix
        :return: list[list[int]] | list[list[float]] 2D data matrix

        Takes a 2D array as an input and reflects it on the diagonal axis.
        Intended to be used so that all rows contain data of the same type 
        (ex. first row is water temperature data).
        """
        rows, cols = len(matrix), len(matrix[0])
        transposed = [[0 for _ in range(rows)] for _ in range(cols)]
        
        for i in range(rows):
            for j in range(cols):
                transposed[j][i] = matrix[i][j]

pca = PCA()
if __name__ == "__main__":
    main()

import { apiClient } from './api';
import { ApiEndpoints } from './api-endpoints';

export class ReviewService {
  /**
   * Get reviews for a station
   */
  static async getReviews(stationId: string): Promise<any[]> {
    try {
      const response = await apiClient.get(ApiEndpoints.REVIEWS.replace(':id', stationId));
      return response.data;
    } catch (error) {
      console.error('Failed to get reviews:', error);
      throw error;
    }
  }

  /**
   * Submit a new review
   */
  static async submitReview(
    stationId: string,
    rating: any,
    reviewText: string
  ): Promise<any> {
    try {
      const response = await apiClient.post(ApiEndpoints.REVIEW_SUBMIT.replace(':id', stationId), {
        rating,
        reviewText,
      });
      return response.data;
    } catch (error) {
      console.error('Failed to submit review:', error);
      throw error;
    }
  }

  /**
   * Get average rating for a station
   */
  static async getAverageRating(stationId: string): Promise<number> {
    try {
      const reviews = await this.getReviews(stationId);
      if (reviews.length === 0) return 0;

      const total = reviews.reduce((sum, review) => sum + review.rating.overall, 0);
      return total / reviews.length;
    } catch (error) {
      console.error('Failed to get average rating:', error);
      return 0;
    }
  }
}

export default ReviewService;

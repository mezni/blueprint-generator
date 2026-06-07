import React, { useState, FormEvent } from 'react';
import { View, Text, StyleSheet, ScrollView, TouchableOpacity, ActivityIndicator, Alert } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useNavigation, useRoute } from '@react-navigation/native';
import { useStations } from '@/hooks/useStations';
import { useAuth } from '@/hooks/useAuth';
import { useFavorites } from '@/hooks/useFavorites';
import { formatDistance } from '@/utils/rtl-utils';
import { ReviewService } from '@/services/review-service';

interface Rating {
  cleanliness: number;
  chargingSpeed: number;
  staff: number;
  overall: number;
}

export function ReviewForm() {
  const navigation = useNavigation();
  const route = useRoute();
  const { isAuthenticated } = useAuth();
  const { data: stations } = useStations();
  const { favorites, toggleFavorite } = useFavorites();
  
  const stationId = (route.params as any)?.stationId || 'unknown';
  const station = stations?.find((s: any) => s.id === stationId);

  const [rating, setRating] = useState<Rating>({
    cleanliness: 5,
    chargingSpeed: 5,
    staff: 5,
    overall: 5,
  });
  const [reviewText, setReviewText] = useState('');
  const [submitting, setSubmitting] = useState(false);

  if (!isAuthenticated) {
    return (
      <View style={styles.container}>
        <Text style={styles.placeholderText}>Please login to submit a review</Text>
      </View>
    );
  }

  if (!station) {
    return (
      <View style={styles.container}>
        <Text style={styles.errorText}>Station not found</Text>
      </View>
    );
  }

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setSubmitting(true);

    try {
      await ReviewService.submitReview(stationId, rating, reviewText);
      Alert.alert('Success', 'Thank you for your review!');
      navigation.goBack();
    } catch (error) {
      Alert.alert('Error', 'Failed to submit review. Please try again.');
    } finally {
      setSubmitting(false);
    }
  };

  const renderRating = (category: keyof Rating) => {
    const stars = rating[category];
    const categories: Record<keyof Rating, string> = {
      cleanliness: 'Cleanliness',
      chargingSpeed: 'Charging Speed',
      staff: 'Staff',
      overall: 'Overall',
    };

    return (
      <View style={styles.ratingContainer}>
        <Text style={styles.ratingLabel}>{categories[category]}</Text>
        <View style={styles.starsContainer}>
          {[1, 2, 3, 4, 5].map((star) => (
            <TouchableOpacity
              key={star}
              onPress={() => setRating({ ...rating, [category]: star })}
              activeOpacity={0.7}
            >
              <Ionicons
                name={star <= stars ? "star" : "star-outline"}
                size={28}
                color={star <= stars ? "#F59E0B" : "#D1D5DB"}
              />
            </TouchableOpacity>
          ))}
        </View>
      </View>
    );
  };

  return (
    <ScrollView style={styles.container} contentContainerStyle={styles.contentContainer}>
      {/* Header */}
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation.goBack()}>
          <Ionicons name="arrow-back" size={24} color="#111827" />
        </TouchableOpacity>
        <Text style={styles.headerTitle}>Rate {station.name}</Text>
        <View style={styles.headerActions} />
      </View>

      {/* Station Info */}
      <View style={styles.stationInfo}>
        <View style={styles.iconContainer}>
          <Ionicons name="car" size={32} color="#2563EB" />
        </View>
        <Text style={styles.stationName}>{station.name}</Text>
        <Text style={styles.stationDescription}>{station.description || 'No description available'}</Text>
      </View>

      {/* Rating Form */}
      <View style={styles.ratingSection}>
        <Text style={styles.sectionTitle}>Your Rating</Text>
        {renderRating('cleanliness')}
        {renderRating('chargingSpeed')}
        {renderRating('staff')}
        {renderRating('overall')}
      </View>

      {/* Review Text */}
      <View style={styles.reviewSection}>
        <Text style={styles.sectionTitle}>Your Review</Text>
        <View style={styles.textAreaContainer}>
          <Text style={styles.textArea} placeholder="Share your experience..." multiline numberOfLines={5} editable={!submitting} />
        </View>
      </View>

      {/* Submit Button */}
      <TouchableOpacity
        style={[styles.submitButton, submitting && styles.submitButtonDisabled]}
        onPress={handleSubmit}
        disabled={submitting}
      >
        {submitting ? (
          <ActivityIndicator size="small" color="#FFFFFF" />
        ) : (
          <Text style={styles.submitButtonText}>Submit Review</Text>
        )}
      </TouchableOpacity>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#FFFFFF',
  },
  contentContainer: {
    paddingBottom: 32,
  },
  header: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingHorizontal: 16,
    paddingVertical: 16,
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  headerTitle: {
    fontSize: 18,
    fontWeight: '700',
    color: '#111827',
  },
  headerActions: {
    padding: 8,
  },
  stationInfo: {
    padding: 16,
    backgroundColor: '#F9FAFB',
    alignItems: 'center',
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  iconContainer: {
    width: 64,
    height: 64,
    borderRadius: 16,
    backgroundColor: '#EFF6FF',
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: 12,
  },
  stationName: {
    fontSize: 18,
    fontWeight: '700',
    color: '#111827',
    marginBottom: 4,
    textAlign: 'center',
  },
  stationDescription: {
    fontSize: 14,
    color: '#6B7280',
    textAlign: 'center',
  },
  ratingSection: {
    padding: 16,
  },
  sectionTitle: {
    fontSize: 16,
    fontWeight: '700',
    color: '#111827',
    marginBottom: 16,
  },
  ratingContainer: {
    marginBottom: 24,
  },
  ratingLabel: {
    fontSize: 14,
    color: '#6B7280',
    marginBottom: 12,
  },
  starsContainer: {
    flexDirection: 'row',
    gap: 8,
  },
  reviewSection: {
    padding: 16,
  },
  textAreaContainer: {
    borderWidth: 1,
    borderColor: '#D1D5DB',
    borderRadius: 8,
    overflow: 'hidden',
  },
  textArea: {
    padding: 12,
    fontSize: 14,
    color: '#111827',
    minHeight: 120,
  },
  submitButton: {
    backgroundColor: '#2563EB',
    padding: 16,
    margin: 16,
    borderRadius: 8,
    alignItems: 'center',
  },
  submitButtonDisabled: {
    backgroundColor: '#9CA3AF',
  },
  submitButtonText: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
  },
  placeholderText: {
    fontSize: 16,
    color: '#6B7280',
    textAlign: 'center',
    marginTop: 40,
  },
  errorText: {
    fontSize: 16,
    color: '#EF4444',
    textAlign: 'center',
    marginTop: 40,
  },
});

import React, { useState } from 'react';
import { StyleSheet, View, Text, TouchableOpacity } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useNavigation, useRoute } from '@react-navigation/native';

export function ReviewForm() {
  const navigation = useNavigation();
  const route = useRoute();

  const station = route.params as any;

  const [rating, setRating] = useState({
    cleanliness: 5,
    chargingSpeed: 5,
    staff: 5,
    overall: 5,
  });
  const [reviewText, setReviewText] = useState('');

  return (
    <View style={styles.container}>
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation.goBack()}>
          <Ionicons name="arrow-back" size={24} color="#111827" />
        </TouchableOpacity>
        <Text style={styles.headerTitle}>Rate {station?.name}</Text>
        <View style={styles.headerActions} />
      </View>

      {station && (
        <View style={styles.content}>
          <View style={styles.iconContainer}>
            <Ionicons name="car" size={32} color="#2563EB" />
          </View>
          <Text style={styles.stationName}>{station.name}</Text>
          <Text style={styles.stationDescription}>
            {station.description || 'No description available'}
          </Text>

          <View style={styles.ratingSection}>
            <Text style={styles.sectionTitle}>Your Rating</Text>
            {renderRating('Cleanliness', 'cleanliness')}
            {renderRating('Charging Speed', 'chargingSpeed')}
            {renderRating('Staff', 'staff')}
            {renderRating('Overall', 'overall')}
          </View>

          <View style={styles.reviewSection}>
            <Text style={styles.sectionTitle}>Your Review</Text>
            <View style={styles.textAreaContainer}>
              <Text style={styles.textArea} placeholder="Share your experience..." />
            </View>
          </View>

          <TouchableOpacity style={styles.submitButton}>
            <Text style={styles.submitButtonText}>Submit Review</Text>
          </TouchableOpacity>
        </View>
      )}
    </View>
  );

  function renderRating(label: string, category: keyof typeof rating) {
    return (
      <View style={styles.ratingContainer}>
        <Text style={styles.ratingLabel}>{label}</Text>
        <View style={styles.starsContainer}>
          {[1, 2, 3, 4, 5].map((star) => (
            <TouchableOpacity
              key={star}
              onPress={() => setRating({ ...rating, [category]: star })}
              activeOpacity={0.7}
            >
              <Ionicons
                name={star <= rating[category] ? "star" : "star-outline"}
                size={28}
                color={star <= rating[category] ? "#F59E0B" : "#D1D5DB"}
              />
            </TouchableOpacity>
          ))}
        </View>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#FFFFFF',
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
  content: {
    padding: 16,
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
    marginBottom: 16,
  },
  ratingSection: {
    marginBottom: 24,
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
    marginBottom: 24,
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
    borderRadius: 8,
    alignItems: 'center',
  },
  submitButtonText: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
  },
});

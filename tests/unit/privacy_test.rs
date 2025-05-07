use doppio::arith;
use doppio::random;
use pretty_assertions::assert_eq;

#[test]
fn test_laplace_noise() {
    let scale = 1.0;
    let noise = random::laplace_noise(scale);
    assert!(noise.is_finite());
}

#[test]
fn test_gaussian_noise() {
    let sigma = 1.0;
    let noise = random::gaussian_noise(sigma);
    assert!(noise.is_finite());
}

#[test]
fn test_histogram_noise() {
    let epsilon = 1.0;
    let noise = random::hist_noise(epsilon);
    assert!(noise.is_finite());
}

#[test]
fn test_arithmetic_operations() {
    let a = 5.0;
    let b = 3.0;
    
    assert_eq!(arith::add(a, b), 8.0);
    assert_eq!(arith::subtract(a, b), 2.0);
    assert_eq!(arith::multiply(a, b), 15.0);
    assert_eq!(arith::divide(a, b), 5.0 / 3.0);
}

#[test]
fn test_privacy_budget() {
    let epsilon = 1.0;
    let delta = 1e-5;
    
    // Test privacy budget calculations
    let budget = arith::PrivacyBudget::new(epsilon, delta);
    assert_eq!(budget.epsilon(), epsilon);
    assert_eq!(budget.delta(), delta);
}

#[test]
fn test_composition() {
    let epsilon1 = 1.0;
    let epsilon2 = 2.0;
    let delta1 = 1e-5;
    let delta2 = 1e-5;
    
    let budget1 = arith::PrivacyBudget::new(epsilon1, delta1);
    let budget2 = arith::PrivacyBudget::new(epsilon2, delta2);
    
    let composed = budget1.compose(&budget2);
    assert_eq!(composed.epsilon(), epsilon1 + epsilon2);
    assert_eq!(composed.delta(), delta1 + delta2);
} 
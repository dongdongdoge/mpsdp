use mpsdp::arith;
use mpsdp::random;
use pretty_assertions::assert_eq;

#[test]
fn test_laplace_noise() {
    let scale = 1.0;
    let noise = random::laplace_noise(scale);
    assert!(noise.is_finite());
}

#[test]
fn test_privacy_budget() {
    let epsilon = 1.0;
    let delta = 1e-5;
    
    let budget = arith::PrivacyBudget::new(epsilon, delta);
    assert_eq!(budget.epsilon(), epsilon);
    assert_eq!(budget.delta(), delta);
} 
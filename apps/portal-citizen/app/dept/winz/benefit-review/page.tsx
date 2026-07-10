import ActionForm from "../ActionForm";

export const metadata = { title: "Submit a Benefit Review — Work and Income — My Gov NZ" };

export default function BenefitReviewPage() {
  return (
    <ActionForm
      actionType="submit-benefit-review"
      title="Submit a Benefit Review"
      description="Let us know about changes to your circumstances so we can review your entitlement."
      inputLabel="Your notes"
      paramKey="notes"
      backHref="/dept/winz"
    />
  );
}

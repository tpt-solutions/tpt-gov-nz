import ActionForm from "../ActionForm";

export const metadata = { title: "Request an Appointment — Work and Income — My Gov NZ" };

export default function RequestAppointmentPage() {
  return (
    <ActionForm
      actionType="request-appointment"
      title="Request an Appointment"
      description="Tell us why you would like to meet with your case worker. We will be in touch to arrange a time."
      inputLabel="Reason for appointment"
      paramKey="reason"
      backHref="/dept/winz"
    />
  );
}

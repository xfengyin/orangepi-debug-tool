name: Feature Request
description: Suggest a new feature or improvement
title: "[Feature] "
labels: ["enhancement", "needs-triage"]
assignees: []
body:
  - type: markdown
    id: description
    attributes:
      label: Feature Description
      description: |
        Describe the feature or improvement you'd like to see.
        
        **What problem does this solve?**
        Explain the use case or problem this feature would address.
        
        **Proposed Solution:**
        Describe what you'd like to happen or how the feature should work.
    validations:
      required: true

  - type: textarea
    id: alternatives
    attributes:
      label: Alternatives Considered
      description: |
        Describe any alternative solutions or features you've considered.
        What are the pros and cons of different approaches?
      placeholder: |
        Alternative A:
        - Pros: ...
        - Cons: ...
        
        Alternative B:
        - Pros: ...
        - Cons: ...
    validations:
      required: false

  - type: dropdown
    id: category
    attributes:
      label: Feature Category
      description: Which category does this feature belong to?
      options:
        - Serial Port / UART
        - GPIO Control
        - PWM Debugging
        - User Interface
        - Documentation
        - Build / CI-CD
        - Performance
        - Cross-Platform Support
        - Plugin System
        - Other
    validations:
      required: false

  - type: input
    id: priority
    attributes:
      label: Priority
    validations:
      required: false

  - type: dropdown
    id: priority-level
    attributes:
      label: Priority Level
      options:
        - Critical (Blocker)
        - High
        - Medium
        - Low
        - Nice to Have
    validations:
      required: false

  - type: textarea
    id: mockups
    attributes:
      label: Mockups / Design Ideas
      description: |
        If you have any mockups, wireframes, or design ideas for this feature, please describe or attach them here.
        
        **UI Changes:**
        - Screens affected
        - New components needed
        - Changes to existing workflows
      placeholder: |
        [Describe your design ideas or attach reference images]
    validations:
      required: false

  - type: textarea
    id: implementation
    attributes:
      label: Implementation Suggestions
      description: |
        Optional: If you have technical suggestions for implementation.
        
        - Suggested API design
        - New commands or plugins needed
        - Dependencies to consider
        - Potential impact on existing features
      placeholder: |
        [Your implementation suggestions]
    validations:
      required: false

  - type: checkboxes
    id: checklist
    attributes:
      label: Checklist
      options:
        - label: I have searched existing issues to avoid duplicates
          required: true
        - label: This feature is not already covered by existing issues
          required: true
        - label: I am willing to help implement this feature if needed
          required: false
        - label: I can provide feedback during the development process
          required: false

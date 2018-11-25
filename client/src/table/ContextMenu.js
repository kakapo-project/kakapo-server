
import React, { Component } from 'react'

import { Popup, Portal } from 'semantic-ui-react'

import Ref from 'semantic-ui-react/dist/commonjs/addons/Ref'
import PortalInner from 'semantic-ui-react/dist/commonjs/addons/Portal/PortalInner'

import _ from 'lodash'

class ContextPortal extends Portal {

  handleContextMenu = (e, ...rest) => {
    const { trigger } = this.props
    const { open } = this.state

    e.preventDefault()

    // Call original event handler
    _.invoke(trigger, 'props.onClick', e, ...rest)

    if (open) {
      this.close(e)
    } else if (!open) {
      this.open(e)
    }
  }

  handleTriggerClick = (e, ...rest) => {
    const { trigger } = this.props
    const { open } = this.state

    e.preventDefault()

    // Call original event handler
    _.invoke(trigger, 'props.onClick', e, ...rest)

    if (open) {
      this.close(e)
    }
  }

  render() {
    const { children, mountNode, trigger } = this.props
    const { open } = this.state

    return (
      <React.Fragment>
        {open && (
          <PortalInner
            mountNode={mountNode}
            onMount={this.handleMount}
            onUnmount={this.handleUnmount}
          >
            {children}
          </PortalInner>
        )}
        {trigger && (
          <Ref innerRef={this.handleTriggerRef}>
            {React.cloneElement(trigger, {
              onContextMenu: this.handleContextMenu,
              onClick: this.handleTriggerClick,
              onMouseLeave: this.handleTriggerMouseLeave,
              onMouseEnter: this.handleTriggerMouseEnter,
            })}
          </Ref>
        )}
      </React.Fragment>
    )
  }

}

class ContextMenu extends Popup {

  static defaultProps = {
    ...Popup.defaultProps,
    on: 'click'
  }

  render() {
    let elem = super.render()
    return (
      <ContextPortal {...elem.props}>
        {elem.props.children}
      </ContextPortal>
    )
  }
}

export default ContextMenu
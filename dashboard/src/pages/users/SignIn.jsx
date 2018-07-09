import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'react-router-redux'
import {Form, Input, message} from 'antd'

import {Submit, formItemLayout} from '../../components/form'
import {client, USERS_SIGN_IN, failed} from '../../request'
import Header from '../../components/Header'
import {TOKEN} from '../../Authorized'
import SharedLinks from './SharedLinks'

const FormItem = Form.Item

class Widget extends Component {
  handleSubmit = (e) => {
    const {push} = this.props
    const {formatMessage} = this.props.intl
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        client.request(USERS_SIGN_IN, values).then((rst) => {
          var token = rst.signInUserByEmail.token
          message.info(formatMessage({id: "flashes.success"}))
          localStorage.setItem(TOKEN, token)
          push('/dashboard/users/logs')
        }).catch(failed)
      }
    })
  }
  render() {
    const {formatMessage} = this.props.intl
    const {getFieldDecorator} = this.props.form
    return (<Form onSubmit={this.handleSubmit}>
      <Header title={{
          id: 'nut.users.sign-in.title'
        }}/>
      <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.email" />} hasFeedback={true}>
        {
          getFieldDecorator('email', {
            rules: [
              {
                type: 'email',
                message: formatMessage({id: "validations.email"})
              }, {
                required: true,
                message: formatMessage({id: "validations.required"})
              }
            ]
          })(<Input/>)
        }
      </FormItem>
      <FormItem {...formItemLayout} label={<FormattedMessage id = "attributes.password" />} hasFeedback={true}>
        {
          getFieldDecorator('password', {
            rules: [
              {
                required: true,
                max: 30,
                min: 6,
                message: formatMessage({id: "validations.password"})
              }
            ]
          })(<Input type="password"/>)
        }
      </FormItem>
      <Submit/>
      <br/>
      <SharedLinks/>
    </Form>)
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  push: PropTypes.func.isRequired
}

export default connect(state => ({}), {push})(Form.create()(injectIntl(Widget)))
